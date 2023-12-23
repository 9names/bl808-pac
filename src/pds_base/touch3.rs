#[doc = "Register `touch3` reader"]
pub struct R(crate::R<TOUCH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch3` writer"]
pub struct W(crate::W<TOUCH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TOUCH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch_channel_cal_en` reader - "]
pub type TOUCH_CHANNEL_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_channel_cal_en` writer - "]
pub type TOUCH_CHANNEL_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `touch_force_value_en` reader - "]
pub type TOUCH_FORCE_VALUE_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_force_value_en` writer - "]
pub type TOUCH_FORCE_VALUE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `touch_data_hys_en` reader - "]
pub type TOUCH_DATA_HYS_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_data_hys_en` writer - "]
pub type TOUCH_DATA_HYS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `touch_lta_en` reader - "]
pub type TOUCH_LTA_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_lta_en` writer - "]
pub type TOUCH_LTA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `touch_lta_order` reader - "]
pub type TOUCH_LTA_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_lta_order` writer - "]
pub type TOUCH_LTA_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH3_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_flt_en` reader - "]
pub type TOUCH_FLT_EN_R = crate::BitReader<bool>;
#[doc = "Field `touch_flt_en` writer - "]
pub type TOUCH_FLT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `touch_flt_order` reader - "]
pub type TOUCH_FLT_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_flt_order` writer - "]
pub type TOUCH_FLT_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH3_SPEC, u8, u8, 3, O>;
#[doc = "Field `touch_self_mutual_sel` reader - "]
pub type TOUCH_SELF_MUTUAL_SEL_R = crate::BitReader<bool>;
#[doc = "Field `touch_self_mutual_sel` writer - "]
pub type TOUCH_SELF_MUTUAL_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `touch_vldo_ccsel` reader - "]
pub type TOUCH_VLDO_CCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `touch_vldo_ccsel` writer - "]
pub type TOUCH_VLDO_CCSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_15_17` reader - "]
pub type RESERVED_15_17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ten_touch` reader - "]
pub type TEN_TOUCH_R = crate::BitReader<bool>;
#[doc = "Field `ten_touch` writer - "]
pub type TEN_TOUCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH3_SPEC, bool, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn touch_channel_cal_en(&self) -> TOUCH_CHANNEL_CAL_EN_R {
        TOUCH_CHANNEL_CAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn touch_force_value_en(&self) -> TOUCH_FORCE_VALUE_EN_R {
        TOUCH_FORCE_VALUE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn touch_data_hys_en(&self) -> TOUCH_DATA_HYS_EN_R {
        TOUCH_DATA_HYS_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_lta_en(&self) -> TOUCH_LTA_EN_R {
        TOUCH_LTA_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn touch_lta_order(&self) -> TOUCH_LTA_ORDER_R {
        TOUCH_LTA_ORDER_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_flt_en(&self) -> TOUCH_FLT_EN_R {
        TOUCH_FLT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn touch_flt_order(&self) -> TOUCH_FLT_ORDER_R {
        TOUCH_FLT_ORDER_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_self_mutual_sel(&self) -> TOUCH_SELF_MUTUAL_SEL_R {
        TOUCH_SELF_MUTUAL_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn touch_vldo_ccsel(&self) -> TOUCH_VLDO_CCSEL_R {
        TOUCH_VLDO_CCSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn reserved_15_17(&self) -> RESERVED_15_17_R {
        RESERVED_15_17_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_touch(&self) -> TEN_TOUCH_R {
        TEN_TOUCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn reserved_19_31(&self) -> RESERVED_19_31_R {
        RESERVED_19_31_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel_cal_en(&mut self) -> TOUCH_CHANNEL_CAL_EN_W<0> {
        TOUCH_CHANNEL_CAL_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_value_en(&mut self) -> TOUCH_FORCE_VALUE_EN_W<1> {
        TOUCH_FORCE_VALUE_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_data_hys_en(&mut self) -> TOUCH_DATA_HYS_EN_W<2> {
        TOUCH_DATA_HYS_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn touch_lta_en(&mut self) -> TOUCH_LTA_EN_W<4> {
        TOUCH_LTA_EN_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_lta_order(&mut self) -> TOUCH_LTA_ORDER_W<5> {
        TOUCH_LTA_ORDER_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_flt_en(&mut self) -> TOUCH_FLT_EN_W<8> {
        TOUCH_FLT_EN_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_flt_order(&mut self) -> TOUCH_FLT_ORDER_W<9> {
        TOUCH_FLT_ORDER_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_self_mutual_sel(&mut self) -> TOUCH_SELF_MUTUAL_SEL_W<12> {
        TOUCH_SELF_MUTUAL_SEL_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_vldo_ccsel(&mut self) -> TOUCH_VLDO_CCSEL_W<13> {
        TOUCH_VLDO_CCSEL_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ten_touch(&mut self) -> TEN_TOUCH_W<18> {
        TEN_TOUCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "touch data process\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch3](index.html) module"]
pub struct TOUCH3_SPEC;
impl crate::RegisterSpec for TOUCH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch3::R](R) reader structure"]
impl crate::Readable for TOUCH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch3::W](W) writer structure"]
impl crate::Writable for TOUCH3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets touch3 to value 0x0660"]
impl crate::Resettable for TOUCH3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0660;
}
