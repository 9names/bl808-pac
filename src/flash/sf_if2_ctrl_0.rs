#[doc = "Register `sf_if2_ctrl_0` reader"]
pub struct R(crate::R<SF_IF2_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF2_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF2_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF2_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if2_ctrl_0` writer"]
pub struct W(crate::W<SF_IF2_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF2_CTRL_0_SPEC>;
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
impl From<crate::W<SF_IF2_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF2_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_sf_if2_rx_inv_sel` reader - "]
pub type SF_CLK_SF_IF2_RX_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_sf_if2_rx_inv_sel` writer - "]
pub type SF_CLK_SF_IF2_RX_INV_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `reserved_3_7` reader - "]
pub type RESERVED_3_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if2_read_dly_n` reader - "]
pub type SF_IF2_READ_DLY_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if2_read_dly_n` writer - "]
pub type SF_IF2_READ_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF2_CTRL_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if2_read_dly_en` reader - "]
pub type SF_IF2_READ_DLY_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_read_dly_en` writer - "]
pub type SF_IF2_READ_DLY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if2_int` reader - "]
pub type SF_IF2_INT_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_int_clr` reader - "]
pub type SF_IF2_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_int_clr` writer - "]
pub type SF_IF2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_int_set` reader - "]
pub type SF_IF2_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_int_set` writer - "]
pub type SF_IF2_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `reserved_19_22` reader - "]
pub type RESERVED_19_22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if2_replace_sf1` reader - "]
pub type SF_IF2_REPLACE_SF1_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_replace_sf1` writer - "]
pub type SF_IF2_REPLACE_SF1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_replace_sf2` reader - "]
pub type SF_IF2_REPLACE_SF2_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_replace_sf2` writer - "]
pub type SF_IF2_REPLACE_SF2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_replace_sf3` reader - "]
pub type SF_IF2_REPLACE_SF3_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_replace_sf3` writer - "]
pub type SF_IF2_REPLACE_SF3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_pad_sel` reader - "]
pub type SF_IF2_PAD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if2_pad_sel` writer - "]
pub type SF_IF2_PAD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF2_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf_if2_bk_swap` reader - "]
pub type SF_IF2_BK_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_bk_swap` writer - "]
pub type SF_IF2_BK_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_bk2_mode` reader - "]
pub type SF_IF2_BK2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_bk2_mode` writer - "]
pub type SF_IF2_BK2_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_bk2_en` reader - "]
pub type SF_IF2_BK2_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_bk2_en` writer - "]
pub type SF_IF2_BK2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if2_bk_sel` reader - "]
pub type SF_IF2_BK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_if2_bk_sel` writer - "]
pub type SF_IF2_BK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF2_CTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_if2_rx_inv_sel(&self) -> SF_CLK_SF_IF2_RX_INV_SEL_R {
        SF_CLK_SF_IF2_RX_INV_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn reserved_3_7(&self) -> RESERVED_3_7_R {
        RESERVED_3_7_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if2_read_dly_n(&self) -> SF_IF2_READ_DLY_N_R {
        SF_IF2_READ_DLY_N_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if2_read_dly_en(&self) -> SF_IF2_READ_DLY_EN_R {
        SF_IF2_READ_DLY_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if2_int(&self) -> SF_IF2_INT_R {
        SF_IF2_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if2_int_clr(&self) -> SF_IF2_INT_CLR_R {
        SF_IF2_INT_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if2_int_set(&self) -> SF_IF2_INT_SET_R {
        SF_IF2_INT_SET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn reserved_19_22(&self) -> RESERVED_19_22_R {
        RESERVED_19_22_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if2_replace_sf1(&self) -> SF_IF2_REPLACE_SF1_R {
        SF_IF2_REPLACE_SF1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_replace_sf2(&self) -> SF_IF2_REPLACE_SF2_R {
        SF_IF2_REPLACE_SF2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_replace_sf3(&self) -> SF_IF2_REPLACE_SF3_R {
        SF_IF2_REPLACE_SF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_if2_pad_sel(&self) -> SF_IF2_PAD_SEL_R {
        SF_IF2_PAD_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if2_bk_swap(&self) -> SF_IF2_BK_SWAP_R {
        SF_IF2_BK_SWAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if2_bk2_mode(&self) -> SF_IF2_BK2_MODE_R {
        SF_IF2_BK2_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if2_bk2_en(&self) -> SF_IF2_BK2_EN_R {
        SF_IF2_BK2_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if2_bk_sel(&self) -> SF_IF2_BK_SEL_R {
        SF_IF2_BK_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sf_if2_rx_inv_sel(&mut self) -> SF_CLK_SF_IF2_RX_INV_SEL_W<2> {
        SF_CLK_SF_IF2_RX_INV_SEL_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_read_dly_n(&mut self) -> SF_IF2_READ_DLY_N_W<8> {
        SF_IF2_READ_DLY_N_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_read_dly_en(&mut self) -> SF_IF2_READ_DLY_EN_W<11> {
        SF_IF2_READ_DLY_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_int_clr(&mut self) -> SF_IF2_INT_CLR_W<17> {
        SF_IF2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_int_set(&mut self) -> SF_IF2_INT_SET_W<18> {
        SF_IF2_INT_SET_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_replace_sf1(&mut self) -> SF_IF2_REPLACE_SF1_W<23> {
        SF_IF2_REPLACE_SF1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_replace_sf2(&mut self) -> SF_IF2_REPLACE_SF2_W<24> {
        SF_IF2_REPLACE_SF2_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_replace_sf3(&mut self) -> SF_IF2_REPLACE_SF3_W<25> {
        SF_IF2_REPLACE_SF3_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_pad_sel(&mut self) -> SF_IF2_PAD_SEL_W<26> {
        SF_IF2_PAD_SEL_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_bk_swap(&mut self) -> SF_IF2_BK_SWAP_W<28> {
        SF_IF2_BK_SWAP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_bk2_mode(&mut self) -> SF_IF2_BK2_MODE_W<29> {
        SF_IF2_BK2_MODE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_bk2_en(&mut self) -> SF_IF2_BK2_EN_W<30> {
        SF_IF2_BK2_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if2_bk_sel(&mut self) -> SF_IF2_BK_SEL_W<31> {
        SF_IF2_BK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if2_ctrl_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if2_ctrl_0](index.html) module"]
pub struct SF_IF2_CTRL_0_SPEC;
impl crate::RegisterSpec for SF_IF2_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if2_ctrl_0::R](R) reader structure"]
impl crate::Readable for SF_IF2_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if2_ctrl_0::W](W) writer structure"]
impl crate::Writable for SF_IF2_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_if2_ctrl_0 to value 0x0002_0004"]
impl crate::Resettable for SF_IF2_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0004;
}
