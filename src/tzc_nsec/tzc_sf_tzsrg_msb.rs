#[doc = "Register `tzc_sf_tzsrg_msb` reader"]
pub struct R(crate::R<TZC_SF_TZSRG_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SF_TZSRG_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SF_TZSRG_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SF_TZSRG_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_sf_tzsrg_msb` writer"]
pub struct W(crate::W<TZC_SF_TZSRG_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SF_TZSRG_MSB_SPEC>;
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
impl From<crate::W<TZC_SF_TZSRG_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SF_TZSRG_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` reader - "]
pub type TZC_SF_TZSRG_R0_END_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` reader - "]
pub type TZC_SF_TZSRG_R0_START_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` reader - "]
pub type TZC_SF_TZSRG_R1_END_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` reader - "]
pub type TZC_SF_TZSRG_R1_START_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` reader - "]
pub type TZC_SF_TZSRG_R2_END_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` reader - "]
pub type TZC_SF_TZSRG_R2_START_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` reader - "]
pub type TZC_SF_TZSRG_R3_END_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_27` reader - "]
pub type RESERVED_27_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` reader - "]
pub type TZC_SF_TZSRG_R3_START_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end_msb(&self) -> TZC_SF_TZSRG_R0_END_MSB_R {
        TZC_SF_TZSRG_R0_END_MSB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start_msb(&self) -> TZC_SF_TZSRG_R0_START_MSB_R {
        TZC_SF_TZSRG_R0_START_MSB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end_msb(&self) -> TZC_SF_TZSRG_R1_END_MSB_R {
        TZC_SF_TZSRG_R1_END_MSB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start_msb(&self) -> TZC_SF_TZSRG_R1_START_MSB_R {
        TZC_SF_TZSRG_R1_START_MSB_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end_msb(&self) -> TZC_SF_TZSRG_R2_END_MSB_R {
        TZC_SF_TZSRG_R2_END_MSB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start_msb(&self) -> TZC_SF_TZSRG_R2_START_MSB_R {
        TZC_SF_TZSRG_R2_START_MSB_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end_msb(&self) -> TZC_SF_TZSRG_R3_END_MSB_R {
        TZC_SF_TZSRG_R3_END_MSB_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved_27(&self) -> RESERVED_27_R {
        RESERVED_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start_msb(&self) -> TZC_SF_TZSRG_R3_START_MSB_R {
        TZC_SF_TZSRG_R3_START_MSB_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_sf_tzsrg_msb\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_sf_tzsrg_msb](index.html) module"]
pub struct TZC_SF_TZSRG_MSB_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_sf_tzsrg_msb::R](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_sf_tzsrg_msb::W](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_MSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_msb to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
